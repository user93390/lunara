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



import Paper from '@mui/material/Paper';
import {FormControl, IconButton, InputLabel, MenuItem, Select, Slider, Typography} from "@mui/material";
import {Close, InsertDriveFile} from "@mui/icons-material";
import {ThemeToggle} from "../components/theme/ThemeToggle.tsx";
import {useCallback, useEffect, useRef, useState} from "react";
import './styles/plugin.scss';

const imageCache = new Map<string, string>();

function CachedImage({src, alt}: { src: string; alt: string }) {
    const [cachedSrc, setCached] = useState<string>(imageCache.get(src) ?? "");

    useEffect(() => {
        if (imageCache.has(src)) {
            setCached(imageCache.get(src)!);
            return;
        }

        const img = new Image();
        img.onload = () => {
            imageCache.set(src, src);
            setCached(src);
        };
        img.src = src;
    }, [src]);

    if (!cachedSrc) {
        return <div className="plugin-icon-placeholder"/>;
    }

    return <img className="plugin-icon" src={cachedSrc} alt={alt}/>;
}

function PluginPage() {
    const [plugins, setPlugins] = useState<Plugin[]>([]);
    const [page, setPage] = useState<number>(1);
    const [loading, setLoading] = useState(false);
    const [selectedPlugin, setSelectedPlugin] = useState<Plugin | null>(null);
    const [versions, setVersions] = useState<string[]>([]);
    const [loadingVersions, setLoadingVersions] = useState(false);
    const [servers, setServers] = useState<string[]>([]);
    const [selectedServer, setSelectedServer] = useState<string>("");
    const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    useEffect(() => {
        fetch("http://localhost:5000/mc/server/list")
            .then(res => res.ok ? res.json() : "[]")
            .then(data => {
                const parsed = typeof data === "string" ? JSON.parse(data) : data;
                const names = parsed.map((s: { name?: string }) => s.name ?? "unknown");
                setServers(names);
                if (names.length > 0) setSelectedServer(names[0]);
            })
            .catch(() => setServers([]));
    }, []);

    const fetchPlugins = useCallback(async (pageNum: number) => {
        setLoading(true);
        const result = await get_plugins(pageNum);
        setPlugins(result);
        setLoading(false);
    }, []);

    useEffect(() => {
        fetchPlugins(page);
    }, [page, fetchPlugins]);

    const handleSliderChange = (_: unknown, value: number | number[]) => {
        const newPage = value as number;

        if (debounceRef.current) {
            clearTimeout(debounceRef.current);
        }

        debounceRef.current = setTimeout(() => {
            setPage(newPage);
        }, 300);
    };

    const handlePluginClick = async (plugin: Plugin) => {
        setSelectedPlugin(plugin);
        setLoadingVersions(true);

        const res = await fetch(`http://localhost:5000/mc/plugin/versions/${plugin.slug}`);
        if (res.ok) {
            setVersions(await res.json());
        } else {
            setVersions([]);
        }

        setLoadingVersions(false);
    };

    const handleDownload = async (slug: string, version: string) => {
        if (!selectedServer) return;

        const url = `http://localhost:5000/mc/server/${selectedServer}/add/${slug}/${version}`;
        await fetch(url);
    };

    return (
        <div className="main">
            <header className="page-header">
                <Typography variant="h4" component="h1" fontWeight={600}>
                    Plugin Marketplace
                </Typography>
                <ThemeToggle/>
            </header>

            <div className="plugin-grid">
                {loading && plugins.length === 0 && (
                    <Typography className="plugin-status">Loading...</Typography>
                )}

                {!loading && plugins.length === 0 && (
                    <Typography className="plugin-status">No plugins found</Typography>
                )}

                {plugins.map((plugin) => (
                    <Paper
                        className="plugin-card"
                        elevation={2}
                        key={plugin.slug}
                        onClick={() => handlePluginClick(plugin)}
                    >
                        <CachedImage src={plugin.icon} alt={plugin.name}/>
                        <div className="plugin-info">
                            <Typography className="plugin-name" fontWeight={600}>
                                {plugin.name}
                            </Typography>
                            <Typography className="plugin-owner" variant="body2">
                                {plugin.owner}
                            </Typography>
                            <div className="plugin-stats">
                                <span>⬇ {plugin.downloads.toLocaleString()}</span>
                                <span>★ {plugin.stars.toLocaleString()}</span>
                            </div>
                        </div>
                    </Paper>
                ))}
            </div>

            {selectedPlugin && (
                <Paper className="version-panel" elevation={3}>
                    <div className="version-panel-header">
                        <CachedImage src={selectedPlugin.icon} alt={selectedPlugin.name}/>
                        <Typography fontWeight={600}>{selectedPlugin.name}</Typography>
                        <IconButton
                            className="version-panel-close"
                            onClick={() => setSelectedPlugin(null)}
                            size="small"
                        >
                            <Close fontSize="small"/>
                        </IconButton>
                    </div>

                    {servers.length > 0 && (
                        <div className="version-panel-server">
                            <FormControl size="small" fullWidth>
                                <InputLabel>Server</InputLabel>
                                <Select
                                    label="Server"
                                    value={selectedServer}
                                    onChange={(e) => setSelectedServer(e.target.value)}
                                >
                                    {servers.map((s) => (
                                        <MenuItem key={s} value={s}>{s}</MenuItem>
                                    ))}
                                </Select>
                            </FormControl>
                        </div>
                    )}

                    <div className="version-list">
                        {loadingVersions && (
                            <Typography className="version-status" variant="body2">
                                Loading versions...
                            </Typography>
                        )}

                        {!loadingVersions && versions.length === 0 && (
                            <Typography className="version-status" variant="body2">
                                No versions found
                            </Typography>
                        )}

                        {!loadingVersions && versions.map((v) => (
                            <div
                                className="version-entry"
                                key={v}
                                onClick={() => handleDownload(selectedPlugin.slug, v)}
                            >
                                <InsertDriveFile className="version-file-icon" fontSize="small"/>
                                <span className="version-file-name">
                                    {selectedPlugin.slug}-{v}.jar
                                </span>
                                <Typography className="version-tag" variant="caption">
                                    {v}
                                </Typography>
                            </div>
                        ))}
                    </div>
                </Paper>
            )}

            <Paper className="page-slider-container" elevation={2}>
                <Typography variant="body2" fontWeight={500}>
                    Page {page}
                </Typography>
                <Slider
                    defaultValue={1}
                    min={1}
                    max={20}
                    step={1}
                    valueLabelDisplay="auto"
                    onChange={handleSliderChange}
                />
            </Paper>
        </div>
    );
}

async function get_plugins(page_number: number): Promise<Plugin[]> {
    const response: Response = await fetch("http://localhost:5000/mc/plugin/trending?trending=" + page_number);

    if (!response.ok) {
        return [];
    }

    const jsonData: TrendingPlugin[] = await response.json();

    return jsonData.map((plugin) => ({
        icon: plugin.avatarUrl,
        name: plugin.name,
        downloads: plugin.stats.downloads,
        stars: plugin.stats.stars,
        owner: plugin.namespace.owner,
        slug: plugin.namespace.slug,
        download_url: plugin.download_url,
        version: plugin.version,
    }));
}

interface TrendingPlugin {
    name: string;
    namespace: {
        owner: string;
        slug: string;
    };
    stats: {
        downloads: number;
        stars: number;
    };
    avatarUrl: string;
    download_url: string;
    version: string;
}

interface Plugin {
    icon: string;
    name: string;
    downloads: number;
    stars: number;
    owner: string;
    slug: string;
    download_url: string;
    version: string;
}

export default PluginPage;