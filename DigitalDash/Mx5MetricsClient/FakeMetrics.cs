using System;

namespace DigitalDash.Mx5MetricsClient;

public class FakeMetrics : IMetrics
{
    private readonly Random _rand = new();
    
    private ushort _rpm = 1000;

    public ushort Rpm
    {
        get
        {
            if (_rpm > 7000)
            {
                _rpm = 1000;
            }

            return _rpm+=9;
        }
    }

    public ushort RedLine => 7000;
    public ushort SpeedKmh => (ushort)_rand.Next(0, 200);
    public byte AcceleratorPedalPositionPct => (byte)_rand.Next(0, 100);
    public byte CalculatedEngineLoadPct => (byte)_rand.Next(0, 100);
    public short EngineCoolantTempC => (short)_rand.Next(0, 150);
    public byte ThrottleValvePositionPct => (byte)_rand.Next(0, 100);
    public short IntakeAirTempC => (short)_rand.Next(0, 40);
    public byte FuelLevelPct => (byte)_rand.Next(0, 100);
    public byte BrakesPct => (byte)_rand.Next(0, 100);
    public ushort FlSpeedKmh => (ushort)_rand.Next(0, 200);
    public ushort FrSpeedKmh => (ushort)_rand.Next(0, 200);
    public ushort RlSpeedKmh => (ushort)_rand.Next(0, 200);
    public ushort RrSpeedKmh => (ushort)_rand.Next(0, 200);
}