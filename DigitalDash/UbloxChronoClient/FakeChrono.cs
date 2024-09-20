using System;

namespace DigitalDash.UbloxChronoClient;

public class FakeChrono : IChrono
{
    private readonly Random _rand = new();

    public uint BestLapTime => (uint)_rand.Next(0, 6000);
    public uint PreviousLapTime => (uint)_rand.Next(0, 6000);
    public uint CurrentLapTime => (uint)_rand.Next(0, 6000);
    public int PreviousSectorDeltaTime => _rand.Next(-1000, 1000);
    public ushort BestLapN => (ushort)_rand.Next(0, 999);
    public ushort CurrentLapN => (ushort)_rand.Next(0, 999);
}