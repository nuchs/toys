namespace Tests
{
    using FluentAssertions;
    using Model;
    using System;
    using Xandos.ViewModels;
    using Xunit;

    public class MarkCommandTests
    {
        [Fact]
        public void BlankSquaresCanBeMarked()
        {
            var sut = new MarkCommand(fakeBoard, 0, 0);

            sut.CanExecute(null).Should().BeTrue();
        }

        [Fact]
        public void NonBlankSquaresCannotBeMarked()
        {
            var sut = new MarkCommand(fakeBoard, 0, 0);

            sut.Execute(null);

            sut.CanExecute(null).Should().BeFalse();
        }

        [Fact]
        public void TheMarkCommandShouldMarkTheBoard()
        {
            var sut = new MarkCommand(fakeBoard, 0, 0);

            sut.Execute(null);

            fakeBoard[0, 0].Should().NotBe(Sigil.Blank);
        }

        private FakeBoard fakeBoard = new FakeBoard();
    }
}