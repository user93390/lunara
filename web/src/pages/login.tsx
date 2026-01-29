import {Button, FormControlLabel, FormGroup, Switch, TextField} from "@mui/material";
import { useNavigate } from 'react-router-dom';

function CredentialLoginPage() {
    const navigate = useNavigate();

    return (
        <>
            <div>
                <TextField id="standard-basic" label="Username"/>
                <TextField id="standard-basic" label="Password" type="password"/>
            </div>

            <div>
                <FormGroup>
                    <FormControlLabel control={<Switch defaultChecked/>} label="Label"/>
                </FormGroup>

                <Button variant="contained" onClick={() => {
                    navigate('/', { replace: true });
                }}>Login</Button>
            </div>
        </>
    );
}

export default CredentialLoginPage