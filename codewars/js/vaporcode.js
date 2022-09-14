// https://www.codewars.com/kata/5966eeb31b229e44eb00007a/train/javascript


function vaporcode(string) {
	string = string.replace(/\s/gm, '').toUpperCase();
	let vapor = '';
	for(let i = 0; i < string.length; ++i) {
		vapor += `${string[i]}  `;
	}

	return vapor.trim();
}


console.log(vaporcode("Lets go to the movies"));