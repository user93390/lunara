/*
 Copyright 2026 seasnail1

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.

 */



import {Button, Link, TextField} from "@mui/material";
import { ThemeToggle } from '../components/theme/mod.ts';
import { useState } from "react";
import './styles/login.scss';

function LoginPage() {

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');

    // Derived state - true when both fields have at least 1 character
    const canLogin: boolean = username.length >= 1 && password.length >= 1;

    return (
        <>
            <ThemeToggle/>

            <div className="welcome">
                <h1>
                    Welcome to Lunara!
                </h1>
            </div>

            <div className="information">
                <TextField
                    id="username-input"
                    label="Username"
                    autoComplete="username"
                    variant="filled"
                    value={username}
                    onChange={(e) => setUsername(e.target.value)}
                />

                <TextField
                    id="filled-password-input"
                    label="Password"
                    type="password"
                    autoComplete="current-password"
                    variant="filled"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                />

                <Button variant="contained" disabled={!canLogin}>
                    Login
                </Button>

                <Link className={"no-acc"}>
                    No account?
                </Link>
            </div>
        </>
    )
}

export default LoginPage