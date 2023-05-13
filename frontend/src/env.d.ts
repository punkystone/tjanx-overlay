interface ImportMetaEnv {
  readonly VITE_WEBSOCKET_PORT: string;
  readonly VITE_SECURE: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
