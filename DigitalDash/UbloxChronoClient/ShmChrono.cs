using System;
using System.IO;
using System.IO.MemoryMappedFiles;

namespace DigitalDash.UbloxChronoClient;

/* shared memory structure
    struct chrono {
        // Timescale is tenths of a second
        uint32_t best_lap_time;
        uint32_t previous_lap_time;
        uint32_t current_lap_time;
        int32_t previous_sector_delta_time;
        uint16_t best_lap_n;
        uint16_t current_lap_n;
    };
 */


public sealed class ShmChrono : IChrono, IDisposable
{
    private readonly MemoryMappedViewAccessor _accessor = MemoryMappedFile
        .CreateFromFile("/dev/shm/ubloxchrono", FileMode.Open, null, 0, MemoryMappedFileAccess.Read)
        .CreateViewAccessor(0, 0, MemoryMappedFileAccess.Read);
    
    public uint BestLapTime => _accessor.ReadUInt32(0);
    public uint PreviousLapTime => _accessor.ReadUInt32(4);
    public uint CurrentLapTime => _accessor.ReadUInt32(8);
    public int PreviousSectorDeltaTime => _accessor.ReadInt32(12);
    public ushort BestLapN => _accessor.ReadUInt16(16);
    public ushort CurrentLapN => _accessor.ReadUInt16(18);
    
    public void Dispose()
    {
        _accessor.Dispose();
    }
}