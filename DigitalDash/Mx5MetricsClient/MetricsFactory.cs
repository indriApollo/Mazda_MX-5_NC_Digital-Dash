using System;

namespace DigitalDash.Mx5MetricsClient;

public static class MetricsFactory
{
    public static IMetrics GetMetrics(bool sharedMemory)
    {
        if (sharedMemory)
        {
            Console.WriteLine("using shm");
            return new ShmMetrics();
        }

        Console.WriteLine("using fake");
        return new FakeMetrics();
    }
}