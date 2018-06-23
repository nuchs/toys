namespace Xandos.Views
{
    using System.Diagnostics;
    using System.Windows;
    using Xandos.ViewModels;

    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        internal MainWindow(MainWindowViewModel viewModel)
        {
            Debug.Assert(viewModel != null);

            DataContext = viewModel;
            InitializeComponent();
        }
    }
}