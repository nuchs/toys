namespace Xandos
{
    using Model;
    using System.Windows;
    using Xandos.ViewModels;
    using Xandos.Views;

    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App : Application
    {
        protected override void OnStartup(StartupEventArgs e)
        {
            base.OnStartup(e);

            BuildObjectGraph().Show();
        }

        private static MainWindow BuildObjectGraph() => new MainWindow(new MainWindowViewModel(new Board()));
    }
}