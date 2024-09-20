using System;
using System.Diagnostics;
using Avalonia.Threading;
using DigitalDash.Mx5MetricsClient;
using DigitalDash.UbloxChronoClient;

namespace DigitalDash;

public class Logic
{
    private readonly DispatcherTimer _highSpeedTimer = new();
    private readonly DispatcherTimer _lowSpeedTimer = new();

    private readonly IMetrics _metrics;
    private readonly IChrono _chrono;
    private readonly Stopwatch _stopwatch = Stopwatch.StartNew();

    public Logic(bool useMetricsShmClient, bool useChronoShmClient)
    {
        _metrics = MetricsFactory.GetMetrics(useMetricsShmClient);
        _chrono = ChronoFactory.GetChrono(useChronoShmClient);
        
        _highSpeedTimer.Interval = TimeSpan.FromMilliseconds(1000d/30d); // 30hz
        _lowSpeedTimer.Interval = TimeSpan.FromMilliseconds(1000); // 1hz
        
        _highSpeedTimer.Start();
        _lowSpeedTimer.Start();
    }

    public const int CoolantAlertThrC = 100;
    public const int RpmWarningThrPct = 80;
    public const int RpmAlertThrPct = 90;
    public const int FuelLevelAlertThrPct = 15;
    public const int WheelSpeedDiffAlertThrKmh = 3;

    public ushort SpeedKmh => _metrics.SpeedKmh;
    public ushort Rpm => _metrics.Rpm;
    public int RpmPct => _metrics.Rpm / (_metrics.RedLine/100);
    public int FuelLevel => _metrics.FuelLevelPct;
    public int Coolant => _metrics.EngineCoolantTempC;
    public short Intake => _metrics.IntakeAirTempC;
    public int Accelerator => _metrics.AcceleratorPedalPositionPct;
    public int Throttle => _metrics.ThrottleValvePositionPct;
    public int EngineLoad => _metrics.CalculatedEngineLoadPct;
    public int Brakes => _metrics.BrakesPct;
    public ushort FlSpeed => _metrics.FlSpeedKmh;
    public ushort FrSpeed => _metrics.FrSpeedKmh;
    public ushort RlSpeed => _metrics.RlSpeedKmh;
    public ushort RrSpeed => _metrics.RrSpeedKmh;
    public string Stint => $"{(int)_stopwatch.Elapsed.TotalMinutes:00}:{_stopwatch.Elapsed.Seconds:00}";

    public int LastSectorDeltaTenths => _chrono.PreviousSectorDeltaTime;
    public uint BestLapTenths => _chrono.BestLapTime;
    public uint LastLapTenths => _chrono.PreviousLapTime;
    public ushort LapCount => _chrono.CurrentLapN;

    public void RegisterHighSpeedRefresh(Action action)
    {
        _highSpeedTimer.Tick += (_, _) => action.Invoke();
    }
    
    public void RegisterLowSpeedRefresh(Action action)
    {
        _lowSpeedTimer.Tick += (_, _) => action.Invoke();
    }
}
