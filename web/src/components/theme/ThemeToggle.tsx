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



import { IconButton } from '@mui/material';
import { DarkMode, LightMode } from '@mui/icons-material';
import { useThemeMode } from './ThemeContext.tsx';

export function ThemeToggle() {
    const { mode, toggleTheme } = useThemeMode();

    return (
        <IconButton
            onClick={toggleTheme}
            color="inherit"
            aria-label={`Switch to ${mode === 'light' ? 'dark' : 'light'} mode`}
            sx={{ ml: 'auto' }}
        >
            {mode === 'dark' ? <LightMode /> : <DarkMode />}
        </IconButton>
    );
}
