{ https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/pascal }

unit even_or_odd;

{$mode objfpc}{$H+}

interface
function EvenOrOdd(Num: Integer): String;

implementation

function EvenOrOdd(Num: Integer): String;
begin
    if Num mod 2 = 0 then
        Result := 'Even'
    else
        Result := 'Odd'
end;
end.
