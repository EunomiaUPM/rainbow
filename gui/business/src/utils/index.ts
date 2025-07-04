export const renameCNTagsForBusiness = (input: string): string => {
    let output: string;
    switch (input) {
        case "REQUESTED":
            output = "REQUESTED"
            break
        case "TERMINATED":
            output = "REJECTED"
            break
        case "FINALIZED":
            output = "APPROVED"
            break
        default:
            output = input
    }
    return output
}