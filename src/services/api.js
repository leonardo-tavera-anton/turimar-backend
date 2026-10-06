const API_URL = import.meta.env.VITE_API_URL || 'https://turimar-backend.onrender.com';

// 1. Verificar Estado de la API y BD
export async function checkHealth() {
  const res = await fetch(`${API_URL}/api/health`);
  return await res.json();
}

// 2. Destinos
export async function getDestinos() {
  const res = await fetch(`${API_URL}/api/v1/destinos`);
  return await res.json();
}

export async function createDestino(data) {
  const res = await fetch(`${API_URL}/api/v1/destinos`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });
  return await res.json();
}