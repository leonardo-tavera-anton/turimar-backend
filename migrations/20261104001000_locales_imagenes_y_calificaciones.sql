INSERT INTO storage.buckets (id, name, public, file_size_limit, allowed_mime_types)
VALUES ('locales', 'locales', TRUE, 5242880, ARRAY['image/jpeg', 'image/png', 'image/webp'])
ON CONFLICT (id) DO UPDATE SET
    public = TRUE,
    file_size_limit = 5242880,
    allowed_mime_types = ARRAY['image/jpeg', 'image/png', 'image/webp'];

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'storage' AND tablename = 'objects'
          AND policyname = 'locales_images_public_read'
    ) THEN
        CREATE POLICY locales_images_public_read ON storage.objects
            FOR SELECT USING (bucket_id = 'locales');
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'storage' AND tablename = 'objects'
          AND policyname = 'locales_images_owner_insert'
    ) THEN
        CREATE POLICY locales_images_owner_insert ON storage.objects
            FOR INSERT TO authenticated
            WITH CHECK (bucket_id = 'locales' AND (storage.foldername(name))[1] = auth.uid()::text);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'storage' AND tablename = 'objects'
          AND policyname = 'locales_images_owner_update'
    ) THEN
        CREATE POLICY locales_images_owner_update ON storage.objects
            FOR UPDATE TO authenticated
            USING (bucket_id = 'locales' AND (storage.foldername(name))[1] = auth.uid()::text)
            WITH CHECK (bucket_id = 'locales' AND (storage.foldername(name))[1] = auth.uid()::text);
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM pg_policies
        WHERE schemaname = 'storage' AND tablename = 'objects'
          AND policyname = 'locales_images_owner_delete'
    ) THEN
        CREATE POLICY locales_images_owner_delete ON storage.objects
            FOR DELETE TO authenticated
            USING (bucket_id = 'locales' AND (storage.foldername(name))[1] = auth.uid()::text);
    END IF;
END $$;
