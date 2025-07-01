import {type ClassValue, clsx} from "clsx"
import {twMerge} from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs))
}

export const generateRandomString = (length: number) => {
    // Define the characters that can be used in the random string.
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    const charactersLength = characters.length;

    // Loop 'length' times to build the string.
    for (let i = 0; i < length; i++) {
        // Get a random index from the characters string and append the character.
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}