# RC: Rapid Copy

RC (Rapid Copy) es una herramienta de copia de archivos ultrarrápida y multiproceso. Diseñada para ofrecer un rendimiento excepcional, RC aprovecha las capacidades modernas de hardware para maximizar la velocidad de copia.

## Características principales

- **Copia ultrarrápida**: Utiliza técnicas de multiproceso y E/S asíncrona para lograr velocidades de copia superiores.
- **Detección inteligente de hardware**: Optimiza automáticamente el rendimiento basándose en el tipo de almacenamiento (SSD, HDD, etc.).
- **Múltiples modos de operación**: Incluye modos de archivo, preservación de atributos, copia recursiva y más (en pocas palabras y en general todo lo que tiene cp de unix).
- **Interfaz de línea de comandos robusta**: Ofrece una amplia gama de opciones para personalizar el comportamiento de copia.
- **Manejo eficiente de errores**: Proporciona información detallada y opciones de depuración para resolver problemas.

## Técnicas avanzadas utilizadas

1. **Multiproceso adaptativo**: RC ajusta dinámicamente el número de hilos basándose en el hardware disponible y el tipo de almacenamiento.
2. **Buffering optimizado**: Utiliza un tamaño de buffer de 8 MB para maximizar el rendimiento de E/S.
3. **Manejo asíncrono de E/S**: Aprovecha las capacidades de Rust para E/S no bloqueante, mejorando el rendimiento en operaciones de disco.

## Instalación

Para instalar el proyecto solo ejecuta este comando si tienes Cargo:

```bash
cargo install --git https://github.com/GoldenHand-Dev/rc
```
o si quieres instalar de otra manera:
```bash
cargo install rapidcopy
```

## Uso básico

```bash
rc [OPCIONES] ORIGEN DESTINO
```

Para obtener una lista completa de opciones, ejecute:

```bash
rc --help
```

## Ejemplos de uso

1. Copiar un archivo:
   ```
   rc informe_ventas.xlsx /home/usuario/Documentos/informe_ventas_copia.xlsx
   ```

2. Copiar un directorio recursivamente:
   ```
   rc -r /home/usuario/Fotos /media/backup/Fotos_2024
   ```

3. Copiar preservando atributos:
   ```
   rc -p contrato_firmado.pdf /home/usuario/Documentos_Legales/contrato_firmado.pdf
   ```

4. Copiar en modo verbose:
   ```
   rc -v presentacion.pptx /home/usuario/Trabajo/presentacion_final.pptx
   ```

5. Copiar sin sobrescribir archivos existentes:
   ```
   rc -n datos_clientes.csv /home/usuario/CRM/nuevos_datos_clientes.csv
   ```

6. Copiar con un número específico de hilos:
   ```
   rc --threads 4 pelicula_4k.mp4 /media/usb/pelicula_4k_copia.mp4
   ```

7. Copiar en modo interactivo:
   ```
   rc -i /home/usuario/Descargas/* /media/externo/Respaldo_Descargas/
   ```

8. Copiar y actualizar solo si el origen es más nuevo:
   ```
   rc -u base_datos.sql /home/usuario/Backups/base_datos_actualizada.sql
   ```

Para una lista completa de opciones, consulte la ayuda del comando (`rc --help`).

## Licencia

RC está licenciado bajo la GNU General Public License v3.0. Consulte el archivo LICENSE para más detalles.

## Contacto

Para reportar problemas o sugerir mejoras, por favor abra un issue en nuestro repositorio o contacteme en 1nu55et1@gmail.com (por favor, si usan gmail en asunto pongan algo como "Rapid Fast issue" o "Rapid Fast recommendation" asi me sera mas facil saber que son).

---

RC: Haciendo que la copia de archivos sea rápida, eficiente y sin complicaciones.

---

Aviso: Este proyecto principalmente es para uso personal, así que no esperen mucha atención al mismo. Pero lo que pasó y la razón de su creación es que cp no existe en Windows y el PowerShell no anda bien en mi computadora de 2 hilos y 8GB de RAM. Así que lo que hice es crear este programa para uso diario. No esperen que sea lo mejor sino algo bueno y simple más que todo, pero como suelo usar bastante cp, al final puse todas las opciones de cp de Unix y quizás en un futuro más.
