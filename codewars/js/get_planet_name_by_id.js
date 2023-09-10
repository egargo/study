// https://www.codewars.com/kata/515e188a311df01cba000003/train/javascript

function getPlanetName(id){
    let planets = {
        1: "Mercury",
        2: "Venus",
        3: "Earth",
        4: "Mars",
        5: "Jupiter",
        6: "Saturn",
        7: "Uranus",
        8: "Neptune",
    };

    return planets[id]
}

console.log(getPlanetName(2))
