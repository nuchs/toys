namespace Model
{
    using System;

    internal class Square
    {
        internal Square(int row, int column)
        {
            description = $"Square({row}, {column}";
        }

        internal Action<Sigil, bool> Notify { get; set; }

        internal Sigil Sigil
        {
            get => sigil;
            set
            {
                sigil = value;
                Notify?.Invoke(sigil, false);
            }
        }

        public override string ToString() => description;

        internal void InWinningLine() => Notify?.Invoke(Sigil, true);

        private readonly string description;
        private Sigil sigil = Sigil.Blank;
    }
}