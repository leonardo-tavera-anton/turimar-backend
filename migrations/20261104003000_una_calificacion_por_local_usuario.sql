DROP POLICY IF EXISTS calificaciones_local_update_own ON public.calificaciones;

WITH ranked_local_ratings AS (
    SELECT
        id,
        ROW_NUMBER() OVER (
            PARTITION BY usuario_id, entidad_id
            ORDER BY created_at DESC NULLS LAST, id DESC
        ) AS position
    FROM public.calificaciones
    WHERE entidad_tipo = 'local'
)
DELETE FROM public.calificaciones AS rating
USING ranked_local_ratings AS ranked
WHERE rating.id = ranked.id
  AND ranked.position > 1;

CREATE UNIQUE INDEX IF NOT EXISTS calificaciones_local_una_por_usuario
    ON public.calificaciones (usuario_id, entidad_id)
    WHERE entidad_tipo = 'local';
