#!/usr/bin/env node

// Verify the exact calculation
const prngValue = 1031784986;
const arrayLength = 583;

const index1 = Math.floor(prngValue * arrayLength / (2 ** 32));
console.log('Method 1:', index1);

const index2 = Math.floor(prngValue * arrayLength / 4294967296);
console.log('Method 2:', index2);

// Exact calculation
const exact = (prngValue * arrayLength) / (2 ** 32);
console.log('Exact value:', exact);
console.log('Floor:', Math.floor(exact));
