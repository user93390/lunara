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



import {createTheme} from '@mui/material/styles';

export const lightTheme = createTheme({
    palette: {
        mode: 'light',
        primary: {main: '#C96480'},
        secondary: {main: '#C96480'},
        background: {default: '#faefef', paper: '#eed9d9'},
    },
});

export const darkTheme = createTheme({
    palette: {
        mode: 'dark',
        primary: {main: '#249dff'},
        secondary: {main: '#ffffff'},
        background: {default: '#010409', paper: '#0d1117'},
    },
});
