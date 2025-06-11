import {Button} from "./ui/button";
import React from "react";
import {cva} from "class-variance-authority";

export const ContractNegotiationActions = ({state, tiny = false}: { state: string, tiny: boolean }) => {
    const h2ClassName = cva("font-semibold mb-4", {
        variants: {
            tiny: {
                true: "hidden",
                false: null
            }
        }
    })
    const containerClassName = cva("", {
        variants: {
            tiny: {
                true: "inline-flex items-center",
                false: "bg-gray-100 p-6"
            }
        }
    })

    return (
        <div className={containerClassName({tiny})}>
            <h2 className={h2ClassName({tiny})}>Actions</h2>
            {state === "REQUESTED" && (<div className="space-x-2">
                <Button>Offer</Button>
                <Button>Agree</Button>
                <Button>Terminate</Button>
            </div>)}
            {state === "OFFERED" && (<div className="space-x-2">
                <Button>Terminate</Button>
            </div>)}
            {state === "ACCEPTED" && (<div className="space-x-2">
                <Button>Agree</Button>
                <Button>Terminate</Button>
            </div>)}
            {state === "AGREED" && (<div className="space-x-2">
                <Button>Terminate</Button>
            </div>)}
            {state === "VERIFIED" && (<div className="space-x-2">
                <Button>Finalize</Button>
                <Button>Terminate</Button>
            </div>)}
            {state === "FINALIZED" && (<div>
                
            </div>)}
            {state === "TERMINATED" && (<div>No further actions</div>)}
        </div>
    )
}