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

    private void Refresh()
    {
        Stint.Text = _logic.Stint;
    }
}