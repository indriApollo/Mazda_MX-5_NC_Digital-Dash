using System;

namespace DigitalDash.UbloxChronoClient;

public static class ChronoFactory
{
    public static IChrono GetChrono(bool useChronoShmClient)
    {
        if (useChronoShmClient)
        {
            Console.WriteLine("using shm chrono client");
            return new ShmChrono();
        }

        Console.WriteLine("using fake chrono");
        return new FakeChrono();
    }
}