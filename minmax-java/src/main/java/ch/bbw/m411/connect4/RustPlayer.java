package ch.bbw.m411.connect4;

public class RustPlayer extends Connect4ArenaMain.DefaultPlayer {
    private native int rustPlay(byte player, Connect4ArenaMain.Stone[] board);

    @Override
    protected int play() {
        byte player = switch (this.myColor) {
            case BLUE -> 0;
            case RED -> 1;
        };
        return this.rustPlay(player, this.board);
    }
}