using System;

namespace Model
{
    public interface IBoard
    {
        bool GameEnded { get; }
        Sigil this[int row, int column] { get; }

        void MarkSquare(int row, int column);

        void MonitorSquare(Action<Sigil, bool> callback, int row, int column);
    }
}