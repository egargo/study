{ https://www.codewars.com/kata/50654ddff44f800200000004/train/pascal }

unit BugFixMultiply;

{$mode objfpc}{$H+}

interface

function Multiply(const A: Integer; const B: Integer): Integer;

implementation

function Multiply(const A: Integer; const B: Integer): Integer;
begin
  Result := A * B;
end;

end.
