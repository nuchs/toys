namespace Tests
{
    using Model;
    using System;

    internal class FakeBoard : IBoard
    {
        public Sigil this[int row, int column] => board[row, column];

        public void MarkSquare(int row, int column)
        {
            board[row, column] = Sigil.Cross;
            callback?.Invoke(Sigil.Cross, GameWon);
        }

        public void MonitorSquare(Action<Sigil, bool> callback, int row, int column) => this.callback = callback;

        public void SetGameWon() => GameWon = true;

        public bool GameEnded { get; set; }

        private Sigil[,] board = new Sigil[,] {
                { Sigil.Blank, Sigil.Blank, Sigil. Blank},
                { Sigil.Blank, Sigil.Blank, Sigil. Blank},
                { Sigil.Blank, Sigil.Blank, Sigil. Blank}
            };

        private Action<Sigil, bool> callback;
        private bool GameWon = false;
    }
}