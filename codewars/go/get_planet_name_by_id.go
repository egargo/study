// https://www.codewars.com/kata/515e188a311df01cba000003/train/go

package main

import "fmt";

func GetPlanetName(ID int) string {
    planets := [8]string{
        "Mercury", "Venus", "Earth", "Mars",
        "Jupiter", "Saturn", "Uranus", "Neptune",
    };

    return planets[ID - 1];
}

func main() {
    var id = 0;
    fmt.Scanf("%d", &id);

    fmt.Println(GetPlanetName(id));
}
