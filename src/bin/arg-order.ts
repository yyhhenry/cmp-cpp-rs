function newArg(title: string) {
    console.log('newArg:', title);
    return title;
}

function consumeArgs(v1: string, v2: string) {
    console.log('consumeValue:', v1, v2);
}

let v1 = newArg('v1');
let v2 = newArg('v2');
consumeArgs(v1, v2);
// newArg: v1
// newArg: v2
// consumeValue: v1 v2

consumeArgs(newArg('v1'), newArg('v2'));
// newArg: v1
// newArg: v2
// consumeValue: v1 v2
