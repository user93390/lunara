/*
 * Copyright 2026 seasnail1
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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