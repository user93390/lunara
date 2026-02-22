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



import {createContext, type ReactNode, useContext, useEffect, useMemo, useState} from 'react';
import {CssBaseline, ThemeProvider} from '@mui/material';
import {darkTheme, lightTheme} from './theme.ts';

interface ThemeModeContextType {
    mode: 'light' | 'dark';
    toggleTheme: () => void;
}

const ThemeModeContext = createContext<ThemeModeContextType>({
    mode: 'light',
    toggleTheme: () => {},
});

export function ThemeModeProvider({children}: { children: ReactNode }) {
    const [mode, setMode] = useState<'light' | 'dark'>('light');

    const toggleTheme = () => setMode(prev => (prev === 'light' ? 'dark' : 'light'));

    const theme = useMemo(() => (mode === 'light' ? lightTheme : darkTheme), [mode]);

    useEffect(() => {
        const root = document.documentElement;
        root.style.setProperty('--primary', theme.palette.primary.main);
        root.style.setProperty('--background', theme.palette.background.default);
        root.style.setProperty('--background-paper', theme.palette.background.paper);
        root.style.setProperty('--text-primary', theme.palette.text.primary);
        root.style.setProperty('--text-secondary', theme.palette.text.secondary);
    }, [theme]);

    const value = useMemo(() => ({mode, toggleTheme}), [mode]);

    return (
        <ThemeModeContext value={value}>
            <ThemeProvider theme={theme}>
                <CssBaseline/>
                {children}
            </ThemeProvider>
        </ThemeModeContext>
    );
}

// eslint-disable-next-line react-refresh/only-export-components
export const useThemeMode = () => useContext(ThemeModeContext);
