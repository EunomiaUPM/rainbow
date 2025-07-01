import React from "react"
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list";
import Heading from "shared/src/components/ui/heading";
import {Badge} from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent";

export const PolicyWrapperShow = ({policy}: { policy: OdrlOffer }) => {
    return (<div>
        <List className=" border border-white/30 bg-white/10 px-4 py-2 rounded-md justify-start">
            <div className="flex">
                <Heading level="h5" className="flex gap-3">
                    <div>Policy with ID</div>
                    <Badge variant="info" className="h-6">
                        {policy["@id"].slice(9, 29) + "[...]"}
                    </Badge>
                </Heading>
            </div>
            <ListItem>
                <ListItemKey>Policy Target</ListItemKey>
                <p>{policy["@type"]}</p>
            </ListItem>
            <ListItem>
                <ListItemKey> Profile</ListItemKey>
                <p className="whitespace-normal">
                    {" "}
                    {JSON.stringify(policy.profile)}
                </p>
            </ListItem>
            <ListItem>
                <ListItemKey> Target</ListItemKey>
                <p> {policy.target.slice(9)}</p>
            </ListItem>
            <div className="h-5"></div>
            <Heading level="h6"> ODRL CONTENT</Heading>
            <div className="flex flex-col gap-2">
                <PolicyComponent policyItem={policy.permission} variant="permission"/>
                <PolicyComponent policyItem={policy.prohibition} variant="prohibition"/>
                <PolicyComponent policyItem={policy.obligation} variant="obligation"/>
            </div>

        </List>

    </div>)
}