namespace DigitalDash.UbloxChronoClient;

public interface IChrono
{
    public uint BestLapTime { get; }
    public uint PreviousLapTime { get; }
    public uint CurrentLapTime { get; }
    public int PreviousSectorDeltaTime { get; }
    public ushort BestLapN { get; }
    public ushort CurrentLapN { get; }
}