import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import App from './App.tsx'
import {ThemeModeProvider} from './components/theme/mod.ts'
import {BrowserRouter} from 'react-router-dom';

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <BrowserRouter>
            <ThemeModeProvider>
                <App/>
            </ThemeModeProvider>
        </BrowserRouter>
    </StrictMode>,
)
