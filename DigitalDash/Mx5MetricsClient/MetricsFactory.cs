using System;

namespace DigitalDash.Mx5MetricsClient;

public static class MetricsFactory
{
    public static IMetrics GetMetrics(bool useMetricsShmClient)
    {
        if (useMetricsShmClient)
        {
            Console.WriteLine("using shm metrics client");
            return new ShmMetrics();
        }

        Console.WriteLine("using fake metrics");
        return new FakeMetrics();
    }
}