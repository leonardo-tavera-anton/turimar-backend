ALTER TABLE public.calificaciones ENABLE ROW LEVEL SECURITY;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'public' AND tablename = 'calificaciones'
          AND policyname = 'calificaciones_local_read_own'
    ) THEN
        CREATE POLICY calificaciones_local_read_own ON public.calificaciones
            FOR SELECT TO authenticated
            USING (usuario_id = auth.uid() AND entidad_tipo = 'local');
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'public' AND tablename = 'calificaciones'
          AND policyname = 'calificaciones_local_insert_own'
    ) THEN
        CREATE POLICY calificaciones_local_insert_own ON public.calificaciones
            FOR INSERT TO authenticated
            WITH CHECK (
                usuario_id = auth.uid()
                AND entidad_tipo = 'local'
                AND estrellas BETWEEN 1 AND 5
            );
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'public' AND tablename = 'calificaciones'
          AND policyname = 'calificaciones_local_update_own'
    ) THEN
        CREATE POLICY calificaciones_local_update_own ON public.calificaciones
            FOR UPDATE TO authenticated
            USING (usuario_id = auth.uid() AND entidad_tipo = 'local')
            WITH CHECK (
                usuario_id = auth.uid()
                AND entidad_tipo = 'local'
                AND estrellas BETWEEN 1 AND 5
            );
    END IF;
END $$;

CREATE OR REPLACE VIEW public.resumen_calificaciones_locales
WITH (security_barrier = TRUE)
AS
SELECT
    entidad_id AS local_id,
    ROUND(AVG(estrellas)::numeric, 1)::float8 AS promedio,
    COUNT(*)::bigint AS total
FROM public.calificaciones
WHERE entidad_tipo = 'local'
GROUP BY entidad_id;

GRANT SELECT ON public.resumen_calificaciones_locales TO anon, authenticated;
