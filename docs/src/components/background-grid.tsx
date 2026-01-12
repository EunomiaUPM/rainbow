// components/background-grid.tsx
import React from 'react';

export function BackgroundGrid() {
    return (
        // Usamos 'fixed' y 'pointer-events-none' para que no interfiera con los clics
        <div className="pointer-events-none fixed inset-0 z-[-1] h-screen w-screen overflow-hidden bg-background">
            {/* 1. BACKGROUND GRID PATTERN */}
            <div
                className="absolute inset-0 h-full w-full bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]"></div>

            {/* 2. GRADIENT BLOB (Opcional: Ajustado para que se vea sutil en docs) */}
            <div
                className="absolute left-0 right-0 top-0 m-auto h-[310px] w-[310px] rounded-full bg-indigo-500/20 opacity-20 blur-[100px]"></div>
        </div>
    );
}