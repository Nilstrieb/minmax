package ch.bbw.m411.connect4;

public class RustPlayer extends Connect4ArenaMain.DefaultPlayer {
    private static native int rustPlay(byte player, byte[] board);

    @Override
    protected int play() {
        byte player = switch (this.myColor) {
            case BLUE -> 0;
            case RED -> 1;
        };
        byte[] boardBuf = new byte[this.board.length];
        for (int i = 0; i < this.board.length; i++) {
            var stone = this.board[i];
            byte value;
            if (stone == null) {
                value = 2;
            } else if (stone == Connect4ArenaMain.Stone.BLUE) {
                value = 0;
            } else {
                value = 1; // red
            }
            boardBuf[i] = value;
        }
        return RustPlayer.rustPlay(player, boardBuf);
    }
}