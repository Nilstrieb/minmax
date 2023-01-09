package ch.bbw.m411.connect4;

public class RustPlayer extends Connect4ArenaMain.DefaultPlayer {
    private static native int rustPlay(byte player, byte[] board);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("rs_wrapper");
    }

    static byte[] encodeBoard(Connect4ArenaMain.Stone[] board) {
        byte[] boardBuf = new byte[board.length];
        for (int i = 0; i < board.length; i++) {
            var stone = board[i];
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
        return boardBuf;
    }

    @Override
    protected int play() {
        byte player = switch (this.myColor) {
            case BLUE -> 0;
            case RED -> 1;
        };
        byte[] boardBuf = RustPlayer.encodeBoard(this.board);
        return RustPlayer.rustPlay(player, boardBuf);
    }
}