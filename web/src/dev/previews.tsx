import {ComponentPreview, Previews} from "@react-buddy/ide-toolbox";
import {PaletteTree} from "./palette";
import CredentialLoginPage from "../pages/login.tsx";

const ComponentPreviews = () => {
    return (
        <Previews palette={<PaletteTree/>}>
            <ComponentPreview path="/CredentialLoginPage">
                <CredentialLoginPage/>
            </ComponentPreview>
        </Previews>
    );
};

export default ComponentPreviews;