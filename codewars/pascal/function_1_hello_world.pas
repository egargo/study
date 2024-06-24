{ https://www.codewars.com/kata/523b4ff7adca849afe000035/train/pascal }

unit function_1_hello_world;

interface
    function Greet (): String;

var
    result: string;

implementation
    function Greet (): String;
        begin
            result := 'hello world!';
        end;
end.
