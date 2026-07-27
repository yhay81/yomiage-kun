export const replaceDetectedEngines = <
  Provider,
  Engine extends { provider: Provider },
>(
  cache: Map<Provider, Engine>,
  engines: Engine[],
  preferredProvider: Provider,
): Engine | undefined => {
  cache.clear();
  for (const engine of engines) cache.set(engine.provider, engine);
  return engines.find((engine) => engine.provider === preferredProvider) ??
    engines.at(0);
};
