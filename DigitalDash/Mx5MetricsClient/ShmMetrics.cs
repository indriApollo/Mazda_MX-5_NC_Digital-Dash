using System;
using System.IO;
using System.IO.MemoryMappedFiles;

namespace DigitalDash.Mx5MetricsClient;

/* shared memory structure
 * #[repr(C)]
 * pub struct Metrics {
 *     rpm: u16,
 *     speed_kmh: u16,
 *     engine_coolant_temp_c: i16,
 *     intake_air_temp_c: i16,
 *     fl_speed_kmh: u16,
 *     fr_speed_kmh: u16,
 *     rl_speed_kmh: u16,
 *     rr_speed_kmh: u16,
 *     accelerator_pedal_position_pct: u8,
 *     calculated_engine_load_pct: u8,
 *     throttle_valve_position_pct: u8,
 *     fuel_level_pct: u8,
 *     brakes_pct: u8
 * }
 */


public sealed class ShmMetrics : IMetrics, IDisposable
{
    private readonly MemoryMappedViewAccessor _accessor = MemoryMappedFile
        .CreateFromFile("/dev/shm/mx5metrics", FileMode.Open, null, 0, MemoryMappedFileAccess.Read)
        .CreateViewAccessor(0, 0, MemoryMappedFileAccess.Read);
    
    public ushort RedLine => 7000;
    public ushort Rpm => _accessor.ReadUInt16(0);
    public ushort SpeedKmh => _accessor.ReadUInt16(2);
    public short EngineCoolantTempC => _accessor.ReadInt16(4);
    public short IntakeAirTempC => _accessor.ReadInt16(6);
    public ushort FlSpeedKmh => _accessor.ReadByte(8);
    public ushort FrSpeedKmh => _accessor.ReadByte(10);
    public ushort RlSpeedKmh => _accessor.ReadByte(12);
    public ushort RrSpeedKmh => _accessor.ReadByte(14);
    public byte AcceleratorPedalPositionPct => _accessor.ReadByte(16);
    public byte CalculatedEngineLoadPct => _accessor.ReadByte(17);
    public byte ThrottleValvePositionPct => _accessor.ReadByte(18);
    public byte FuelLevelPct => _accessor.ReadByte(19);
    public byte BrakesPct => _accessor.ReadByte(20);
    
    public void Dispose()
    {
        _accessor.Dispose();
    }
}