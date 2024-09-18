using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class WheelSpeeds : UserControl
{
    private readonly Logic _logic = App.Logic;
    
    public WheelSpeeds()
    {
        InitializeComponent();
        
        _logic.RegisterHighSpeedRefresh(Refresh);
    }

    private void Refresh()
    {
        var flSpeed = _logic.FlSpeed;
        var frSpeed = _logic.FrSpeed;
        var rlSpeed = _logic.RlSpeed;
        var rrSpeed = _logic.RrSpeed;
        
        Fl.Text = flSpeed.ToString();
        Fr.Text = frSpeed.ToString();
        Rl.Text = rlSpeed.ToString();
        Rr.Text = rrSpeed.ToString();

        var fDiff = flSpeed - frSpeed;

        switch (fDiff)
        {
            case >= Logic.WheelSpeedDiffAlertThrKmh:
                Fl.Foreground = ColorPalette.Red;
                Fr.Foreground = ColorPalette.White;
                break;
            case <= -Logic.WheelSpeedDiffAlertThrKmh:
                Fr.Foreground = ColorPalette.Red;
                Fl.Foreground = ColorPalette.White;
                break;
            default:
                Fl.Foreground = ColorPalette.White;
                Fr.Foreground = ColorPalette.White;
                break;
        }
        
        var rDiff = rlSpeed - rrSpeed;

        switch (rDiff)
        {
            case >= Logic.WheelSpeedDiffAlertThrKmh:
                Rl.Foreground = ColorPalette.Red;
                Rr.Foreground = ColorPalette.White;
                break;
            case <= -Logic.WheelSpeedDiffAlertThrKmh:
                Rr.Foreground = ColorPalette.Red;
                Rl.Foreground = ColorPalette.White;
                break;
            default:
                Rl.Foreground = ColorPalette.White;
                Rr.Foreground = ColorPalette.White;
                break;
        }
    }
}