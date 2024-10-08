#include <stdio.h>
#include <stdlib.h>
#include <sys/epoll.h>
#include <unistd.h>
#include <signal.h>
#include <sys/signalfd.h>
#include <sys/mman.h>
#include <fcntl.h>

#include "chrono/chrono.h"
#include "ublox/ublox.h"

#define _RUN_TESTS

#ifdef RUN_TESTS
#include "tests/ublox_tests.h"
#include "tests/chrono_tests.h"
#endif

#define SERIAL_PORT_NAME   "/dev/serial/by-id/usb-u-blox_AG_-_www.u-blox.com_u-blox_GNSS_receiver-if00"
#define SERIAL_BAUD_RATE   38400
#define SHM_NAME           "/ubloxchrono"
#define EPOLL_SINGLE_EVENT 1

static struct chrono* setup_shm() {
    const int fd = shm_open(SHM_NAME, O_CREAT | O_RDWR, 0755);
    if (fd < 0) {
        perror("shm_open");
        exit(EXIT_FAILURE);
    }

    if (ftruncate(fd, sizeof(struct chrono)) < 0) {
        perror("ftruncate");
        exit(EXIT_FAILURE);
    }

    struct chrono *chrono = mmap(NULL, sizeof(struct chrono), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (chrono == MAP_FAILED) {
        perror("mmap");
        exit(EXIT_FAILURE);
    }

    close(fd);

    return chrono;
}


static int setup_signal_handler() {
    sigset_t mask;

    sigemptyset(&mask);
    sigaddset(&mask, SIGINT);
    sigaddset(&mask, SIGTERM);

    // Block signals so that they aren't handled
    // according to their default dispositions
    if (sigprocmask(SIG_BLOCK, &mask, NULL) == -1) {
        perror("sigprocmask");
        exit(EXIT_FAILURE);
    }

    const int fd = signalfd(-1, &mask, 0);
    if (fd == -1) {
        perror("signalfd");
        exit(EXIT_FAILURE);
    }

    return fd;
}

static void handle_signal(const int fd) {
    struct signalfd_siginfo siginfo;

    const ssize_t s = read(fd, &siginfo, sizeof(siginfo));
    if (s != sizeof(siginfo)) {
        perror("read signalfd");
        exit(EXIT_FAILURE);
    }

    if (siginfo.ssi_signo == SIGINT) {
        printf("Got SIGINT\n");
    } else if (siginfo.ssi_signo == SIGTERM) {
        printf("Got SIGTERM\n");
    } else {
        printf("Unexpected signal %d\n", siginfo.ssi_signo);
    }
}

static void epoll_add_fd(const int epfd, const int fd) {
    struct epoll_event event;
    event.events = EPOLLIN;
    event.data.fd = fd;

    if(epoll_ctl(epfd, EPOLL_CTL_ADD, fd, &event) < 0) {
        perror("epoll_ctl");
        exit(EXIT_FAILURE);
    }
}

static int setup_epoll(const int signalfd_fd, const int ublox_fd) {
    const int fd = epoll_create1(0);
    if (fd < 0) {
        perror("epoll_create1");
        exit(EXIT_FAILURE);
    }

    epoll_add_fd(fd, signalfd_fd);
    epoll_add_fd(fd, ublox_fd);

    return fd;
}

int main(void)
{
    #ifdef RUN_TESTS
    run_ublox_tests();
    run_chrono_tests();
    exit(EXIT_SUCCESS);
    #endif

    struct chrono *chrono = setup_shm();
    struct gate_segment segments[] = {
      {.a = {.lon = 1, .lat = 1}, .b = {.lon = 2, .lat = 2} },
      {.a = {.lon = 1, .lat = 1}, .b = {.lon = 2, .lat = 2} },
      {.a = {.lon = 1, .lat = 1}, .b = {.lon = 2, .lat = 2} },
    };
    struct sector_gates gates = {.current_index = 0, .count = 3, .segments = segments };
    struct chrono_context chronoContext = {.chrono = chrono, .gates = &gates};

    const int signalfd_fd = setup_signal_handler();

    set_ublox_position_callback(&handle_position, 3000, &chronoContext);

    printf("Setting up serial port %s@%d\n", SERIAL_PORT_NAME, SERIAL_BAUD_RATE);
    const int ublox_fd = setup_ublox_port(SERIAL_PORT_NAME, SERIAL_BAUD_RATE);
    if (ublox_fd < 0) exit(EXIT_FAILURE);

    printf("Configuring ublox ...\n");
    configure_ublox(ublox_fd);

    request_ublox_version(ublox_fd);

    const int epoll_fd = setup_epoll(signalfd_fd, ublox_fd);

    printf("Ready at /dev/shm%s\n", SHM_NAME);

    while(1) {
        struct epoll_event epoll_events[EPOLL_SINGLE_EVENT];

        if (epoll_wait(epoll_fd, epoll_events, EPOLL_SINGLE_EVENT, -1) != EPOLL_SINGLE_EVENT) {
            perror("epoll_wait");
            exit(EXIT_FAILURE);
        }

        if (!(epoll_events[0].events & EPOLLIN)) {
            fprintf(stderr, "Expected EPOLLIN, got %d\n", epoll_events[0].events);
            break;
        }

        if (epoll_events[0].data.fd == ublox_fd) {
            handle_incoming_ublox_msg(ublox_fd);
        }
        else if (epoll_events[0].data.fd == signalfd_fd) {
            handle_signal(signalfd_fd);
            break;
        }
        else {
            fprintf(stderr, "Unexpected epoll event fd %d\n", epoll_events[0].data.fd);
            break;
        }
    }

    printf("Shutting down ....\n");

    close(epoll_fd);
    close(signalfd_fd);
    close_ublox_port(ublox_fd);
    shm_unlink(SHM_NAME);

    printf("Bye :)\n");

    return 0;
}
