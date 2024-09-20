using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class Chrono : UserControl
{
    private readonly Logic _logic = App.Logic;
    
    public Chrono()
    {
        InitializeComponent();
        
        _logic.RegisterHighSpeedRefresh(Refresh);
    }

    private static string TenthsToTimeString(uint tenths)
    {
        var m = tenths / 600;
        var s = (tenths % 600) / 10;
        var t = (tenths % 600) % 10;

        return $"{m:D2}:{s:D2}:{t}";
    }

    private void Refresh()
    {
        var lastSectorDeltaTenths = _logic.LastSectorDeltaTenths;
        Delta.Foreground = lastSectorDeltaTenths > 0 ? ColorPalette.Red : ColorPalette.Green;
        Delta.Text = lastSectorDeltaTenths.ToString();
        
        Stint.Text = _logic.Stint;
        
        Best.Text = TenthsToTimeString(_logic.BestLapTenths);
        Last.Text = TenthsToTimeString(_logic.LastLapTenths);
        Count.Text = _logic.LapCount.ToString("D3");
    }
}