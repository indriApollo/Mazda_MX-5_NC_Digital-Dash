namespace DigitalDash.Mx5MetricsClient;

public interface IMetrics
{
    public ushort RedLine { get; }
    public ushort Rpm { get; }
    public ushort SpeedKmh { get; }
    public short EngineCoolantTempC { get; }
    public short IntakeAirTempC { get; }
    public ushort FlSpeedKmh { get; }
    public ushort FrSpeedKmh { get; }
    public ushort RlSpeedKmh { get; }
    public ushort RrSpeedKmh { get; }
    public byte AcceleratorPedalPositionPct { get; }
    public byte CalculatedEngineLoadPct { get; }
    public byte ThrottleValvePositionPct { get; }
    public byte FuelLevelPct { get; }
    public byte BrakesPct { get; }
}