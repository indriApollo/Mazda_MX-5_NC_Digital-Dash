using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class RpmBar : UserControl
{
    private const int BarLines = 33;
    private const float BarLinePct = BarLines / 100f;
    
    private readonly Logic _logic = App.Logic;
    
    public RpmBar()
    {
        InitializeComponent();

        Bg.Text = new string('|', BarLines);
        
        _logic.RegisterHighSpeedRefresh(Refresh);
    }
    
    private void Refresh()
    {
        var rpmPct = _logic.RpmPct;

        Bar.Foreground = rpmPct switch
        {
            < Logic.RpmWarningThrPct => ColorPalette.Green,
            < Logic.RpmAlertThrPct => ColorPalette.Yellow,
            _ => ColorPalette.Red
        };
        Bar.Text = new string('|', (int)(rpmPct * BarLinePct));
    }
}