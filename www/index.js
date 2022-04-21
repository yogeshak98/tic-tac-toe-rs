import {Board, Cell, Status, DifficultyLevel} from 'tic-tac-toe';
import {memory} from 'tic-tac-toe/tic_tac_toe_bg.wasm';

class TicTacToe{
    constructor(){
        this.CELL_SIZE = 100;
        this.size = 3;
        this.num_cells = this.size * this.size;
        this.GRID_COLOR = "#444444";
        this.player_1 = "×";
        this.player_2 = "o";
        this.current_player = Cell.Player1;
        this.document = document;

        this.board = Board.new();

        this.canvas = this.getCanvas();
        this.ctx = this.canvas.getContext('2d');
        this.ctx.font = '1 60px Verdana';
        this.ctx.textAlign="center"; 
        this.ctx.textBaseline = "middle";
        this.animationId = null;

        this.endgame = false;
        this.addCellToggleEventListener();
        this.addLevelToggleEventListener();
        this.addSymbolToggleEventListener();
        this.addFirstPlayerEventListener();
    }

    getCanvas(){
        const canvas = this.document.getElementById("tic-tac-toe-canvas");
        canvas.height = (this.CELL_SIZE + 1) * this.size + 1;
        canvas.width = (this.CELL_SIZE + 1) * this.size + 1;
        return canvas;
    }

    play(){
        this.renderLoop();
    }

    drawGrid(){
        this.ctx.beginPath();
        this.ctx.strokeStyle = this.GRID_COLOR;
    
        // vertical liines
        for (let i = 0; i <= this.size; i++){
            this.ctx.moveTo(i * (this.CELL_SIZE + 1) + 1, 0);
            this.ctx.lineTo(i * (this.CELL_SIZE + 1) + 1, (this.CELL_SIZE + 1) * this.size + 1);
        }
    
        // horizontal lines
        for (let i = 0; i <= this.size; i++){
            this.ctx.moveTo(0                                    , i * (this.CELL_SIZE + 1) + 1);
            this.ctx.lineTo((this.CELL_SIZE + 1) * this.size + 1 , i * (this.CELL_SIZE + 1) + 1);
        }
    
        this.ctx.stroke();
    }

    getIndex(row, column) {
        return row * this.size + column;
    };

    renderLoop = () => {

        this.drawGrid();
        this.drawCells();
    
        this.animationId = requestAnimationFrame(this.renderLoop);
    }

    addSymbolToggleEventListener(){
        this.document.getElementById("difficultyLevel").addEventListener("change", (event) => {
            this.board.set_difficulty(DifficultyLevel[this.document.getElementById("difficultyLevel").value])
        });
    }

    addLevelToggleEventListener() {
        this.document.getElementById("symbolToggler").addEventListener("change", (event) => {
            this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
            [this.player_1, this.player_2] = [this.player_2, this.player_1];
        });
    }

    addFirstPlayerEventListener(){
        this.document.getElementById("firstPlayer").addEventListener("change", (event) => {
            this.board.reset();
            this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
            if (this.document.getElementById("firstPlayer").value == "AI"){
                this.makeFirstMove();
            }
        });
    }

    makeFirstMove(){
        this.next_player = this.current_player === Cell.Player1 ? Cell.Player2 : Cell.Player1;
        this.board.make_next_valid_move(this.next_player);
    }

    addCellToggleEventListener() {
        this.canvas.addEventListener("click", event => {
            const boundingRect = this.canvas.getBoundingClientRect();
        
            const scaleX = this.canvas.width / boundingRect.width;
            const scaleY = this.canvas.height / boundingRect.height;
        
            const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
            const canvasTop = (event.clientY - boundingRect.top) * scaleY;
        
            const row = Math.min(Math.floor(canvasTop / (this.CELL_SIZE + 1)), this.size - 1);
            const col = Math.min(Math.floor(canvasLeft / (this.CELL_SIZE + 1)), this.size - 1);

            const idx = this.getIndex(row, col);
            if (!this.board.is_empty_cell(idx)){
                return;
            }
            this.board.set_cell(idx, this.current_player);
            this.fillCellText(row, col, this.current_player);
            this.next_player = this.current_player === Cell.Player1 ? Cell.Player2 : Cell.Player1;
            this.board.make_next_valid_move(this.next_player);
            const winner = this.board.get_game_status();

            if (winner === Status.Human || winner === Status.Computer || winner === Status.Draw){
                this.announceWinner(winner);
                return;
            }
        });
    }

    announceWinner(winner){
        if (winner === Status.Human){
            alert("You are the winner!!");
        }
        else if (winner === Status.Computer){
            alert("You lost!!");
        }
        else if (winner === Status.Draw){
            alert("Game is a draw!!");
        }
        this.board.reset();
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    }

    fillCellText(row, col, player){
        this.ctx.fillText(
            player === Cell.Player1 ? this.player_1 : (player === Cell.Player2 ? this.player_2: ""),                    
            col * (this.CELL_SIZE + 1) + 1 + 50,
            row * (this.CELL_SIZE + 1) + 1 + 50
        );
    }

    drawCells(){
        const cellsPtr = this.board.get_cells_ptr();
        const cells = new Uint8Array(memory.buffer, cellsPtr, this.num_cells);

        this.ctx.beginPath();
        for(let row = 0; row < this.size; row++){
            for(let col = 0; col < this.size; col++){
                const idx = this.getIndex(row, col);

                this.ctx.fillText(
                    cells[idx] === Cell.Player1 ? this.player_1 : (cells[idx] === Cell.Player2 ? this.player_2: ""),                    
                    col * (this.CELL_SIZE + 1) + 1 + 50,
                    row * (this.CELL_SIZE + 1) + 1 + 50,
                );
            }
        }
        this.ctx.stroke();
    }
}

const tictactoe = new TicTacToe();
tictactoe.play();    
