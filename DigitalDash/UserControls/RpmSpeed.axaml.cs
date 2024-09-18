using Avalonia.Controls;

namespace DigitalDash.UserControls;

public partial class RpmSpeed : UserControl
{
    private readonly Logic _logic = App.Logic;
    
    public RpmSpeed()
    {
        InitializeComponent();

        _logic.RegisterHighSpeedRefresh(Refresh);
    }

    private void Refresh()
    {
        Rpm.Text = _logic.Rpm.ToString();
        Speed.Text = _logic.SpeedKmh.ToString();
    }
}