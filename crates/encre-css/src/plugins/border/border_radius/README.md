Utilities for controlling the border radius of an element.

<style>
#border-radius-table > tr td:nth-child(3) {
  position: relative;
}

#border-radius-table > tr {
  height: 6.5rem;
}

#border-radius-table tr td:nth-child(3) div {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  border: 4px solid rgb(209 213 219);
  margin: 1rem;
}
</style>

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
      <th style="text-align: center; width: 6.5rem;">Preview</th>
    </tr>
  </thead>
  <tbody id="border-radius-table">
     <tr><td>rounded-none</td><td>border-radius: 0px;</td><td><div style="border-radius: 0px;"></div></td></tr>
     <tr><td>rounded-xs</td><td>border-radius: 0.125rem;</td><td><div style="border-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-sm</td><td>border-radius: 0.25rem;</td><td><div style="border-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-md</td><td>border-radius: 0.375rem;</td><td><div style="border-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-lg</td><td>border-radius: 0.5rem;</td><td><div style="border-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-xl</td><td>border-radius: 0.75rem;</td><td><div style="border-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-2xl</td><td>border-radius: 1rem;</td><td><div style="border-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-3xl</td><td>border-radius: 1.5rem;</td><td><div style="border-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-full</td><td>border-radius: 9999px;</td><td><div style="border-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-s-none</td><td>border-start-start-radius: 0px;<br>border-end-start-radius: 0px;</td><td><div style="border-top-left-radius: 0px;border-top-right-radius: 0px;"></div></td></tr>
     <tr><td>rounded-s-xs</td><td>border-start-start-radius: 0.125rem;<br>border-end-start-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;border-top-right-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-s-sm</td><td>border-start-start-radius: 0.25rem;<br>border-end-start-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;border-top-right-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-s-md</td><td>border-start-start-radius: 0.375rem;<br>border-end-start-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;border-top-right-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-s-lg</td><td>border-start-start-radius: 0.5rem;<br>border-end-start-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;border-top-right-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-s-xl</td><td>border-start-start-radius: 0.75rem;<br>border-end-start-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;border-top-right-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-s-2xl</td><td>border-start-start-radius: 1rem;<br>border-end-start-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;border-top-right-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-s-3xl</td><td>border-start-start-radius: 1.5rem;<br>border-end-start-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;border-top-right-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-s-full</td><td>border-start-start-radius: 9999px;<br>border-end-start-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;border-top-right-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-e-none</td><td>border-start-end-radius: 0px;<br>border-end-end-radius: 0px;</td><td><div style="border-top-left-radius: 0px;border-top-right-radius: 0px;"></div></td></tr>
     <tr><td>rounded-e-xs</td><td>border-start-end-radius: 0.125rem;<br>border-end-end-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;border-top-right-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-e-sm</td><td>border-start-end-radius: 0.25rem;<br>border-end-end-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;border-top-right-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-e-md</td><td>border-start-end-radius: 0.375rem;<br>border-end-end-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;border-top-right-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-e-lg</td><td>border-start-end-radius: 0.5rem;<br>border-end-end-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;border-top-right-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-e-xl</td><td>border-start-end-radius: 0.75rem;<br>border-end-end-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;border-top-right-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-e-2xl</td><td>border-start-end-radius: 1rem;<br>border-end-end-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;border-top-right-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-e-3xl</td><td>border-start-end-radius: 1.5rem;<br>border-end-end-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;border-top-right-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-e-full</td><td>border-start-end-radius: 9999px;<br>border-end-end-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;border-top-right-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-t-none</td><td>border-top-left-radius: 0px;<br>border-top-right-radius: 0px;</td><td><div style="border-top-left-radius: 0px;border-top-right-radius: 0px;"></div></td></tr>
     <tr><td>rounded-t-xs</td><td>border-top-left-radius: 0.125rem;<br>border-top-right-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;border-top-right-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-t-sm</td><td>border-top-left-radius: 0.25rem;<br>border-top-right-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;border-top-right-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-t-md</td><td>border-top-left-radius: 0.375rem;<br>border-top-right-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;border-top-right-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-t-lg</td><td>border-top-left-radius: 0.5rem;<br>border-top-right-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;border-top-right-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-t-xl</td><td>border-top-left-radius: 0.75rem;<br>border-top-right-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;border-top-right-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-t-2xl</td><td>border-top-left-radius: 1rem;<br>border-top-right-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;border-top-right-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-t-3xl</td><td>border-top-left-radius: 1.5rem;<br>border-top-right-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;border-top-right-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-t-full</td><td>border-top-left-radius: 9999px;<br>border-top-right-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;border-top-right-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-r-none</td><td>border-top-right-radius: 0px;<br>border-bottom-right-radius: 0px;</td><td><div style="border-top-right-radius: 0px;border-bottom-right-radius: 0px;"></div></td></tr>
     <tr><td>rounded-r-xs</td><td>border-top-right-radius: 0.125rem;<br>border-bottom-right-radius: 0.125rem;</td><td><div style="border-top-right-radius: 0.125rem;border-bottom-right-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-r-sm</td><td>border-top-right-radius: 0.25rem;<br>border-bottom-right-radius: 0.25rem;</td><td><div style="border-top-right-radius: 0.25rem;border-bottom-right-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-r-md</td><td>border-top-right-radius: 0.375rem;<br>border-bottom-right-radius: 0.375rem;</td><td><div style="border-top-right-radius: 0.375rem;border-bottom-right-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-r-lg</td><td>border-top-right-radius: 0.5rem;<br>border-bottom-right-radius: 0.5rem;</td><td><div style="border-top-right-radius: 0.5rem;border-bottom-right-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-r-xl</td><td>border-top-right-radius: 0.75rem;<br>border-bottom-right-radius: 0.75rem;</td><td><div style="border-top-right-radius: 0.75rem;border-bottom-right-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-r-2xl</td><td>border-top-right-radius: 1rem;<br>border-bottom-right-radius: 1rem;</td><td><div style="border-top-right-radius: 1rem;border-bottom-right-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-r-3xl</td><td>border-top-right-radius: 1.5rem;<br>border-bottom-right-radius: 1.5rem;</td><td><div style="border-top-right-radius: 1.5rem;border-bottom-right-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-r-full</td><td>border-top-right-radius: 9999px;<br>border-bottom-right-radius: 9999px;</td><td><div style="border-top-right-radius: 9999px;border-bottom-right-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-b-none</td><td>border-bottom-right-radius: 0px;<br>border-bottom-left-radius: 0px;</td><td><div style="border-bottom-right-radius: 0px;border-bottom-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-b-xs</td><td>border-bottom-right-radius: 0.125rem;<br>border-bottom-left-radius: 0.125rem;</td><td><div style="border-bottom-right-radius: 0.125rem;border-bottom-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-b-sm</td><td>border-bottom-right-radius: 0.25rem;<br>border-bottom-left-radius: 0.25rem;</td><td><div style="border-bottom-right-radius: 0.25rem;border-bottom-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-b-md</td><td>border-bottom-right-radius: 0.375rem;<br>border-bottom-left-radius: 0.375rem;</td><td><div style="border-bottom-right-radius: 0.375rem;border-bottom-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-b-lg</td><td>border-bottom-right-radius: 0.5rem;<br>border-bottom-left-radius: 0.5rem;</td><td><div style="border-bottom-right-radius: 0.5rem;border-bottom-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-b-xl</td><td>border-bottom-right-radius: 0.75rem;<br>border-bottom-left-radius: 0.75rem;</td><td><div style="border-bottom-right-radius: 0.75rem;border-bottom-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-b-2xl</td><td>border-bottom-right-radius: 1rem;<br>border-bottom-left-radius: 1rem;</td><td><div style="border-bottom-right-radius: 1rem;border-bottom-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-b-3xl</td><td>border-bottom-right-radius: 1.5rem;<br>border-bottom-left-radius: 1.5rem;</td><td><div style="border-bottom-right-radius: 1.5rem;border-bottom-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-b-full</td><td>border-bottom-right-radius: 9999px;<br>border-bottom-left-radius: 9999px;</td><td><div style="border-bottom-right-radius: 9999px;border-bottom-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-l-none</td><td>border-top-left-radius: 0px;<br>border-bottom-left-radius: 0px;</td><td><div style="border-top-left-radius: 0px;border-bottom-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-l-xs</td><td>border-top-left-radius: 0.125rem;<br>border-bottom-left-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;border-bottom-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-l-sm</td><td>border-top-left-radius: 0.25rem;<br>border-bottom-left-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;border-bottom-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-l-md</td><td>border-top-left-radius: 0.375rem;<br>border-bottom-left-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;border-bottom-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-l-lg</td><td>border-top-left-radius: 0.5rem;<br>border-bottom-left-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;border-bottom-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-l-xl</td><td>border-top-left-radius: 0.75rem;<br>border-bottom-left-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;border-bottom-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-l-2xl</td><td>border-top-left-radius: 1rem;<br>border-bottom-left-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;border-bottom-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-l-3xl</td><td>border-top-left-radius: 1.5rem;<br>border-bottom-left-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;border-bottom-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-l-full</td><td>border-top-left-radius: 9999px;<br>border-bottom-left-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;border-bottom-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-ss-none</td><td>border-start-start-radius: 0px;</td><td><div style="border-top-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-ss-xs</td><td>border-start-start-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-ss-sm</td><td>border-start-start-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-ss-md</td><td>border-start-start-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-ss-lg</td><td>border-start-start-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-ss-xl</td><td>border-start-start-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-ss-2xl</td><td>border-start-start-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-ss-3xl</td><td>border-start-start-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-ss-full</td><td>border-start-start-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-se-none</td><td>border-start-end-radius: 0px;</td><td><div style="border-top-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-se-xs</td><td>border-start-end-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-se-sm</td><td>border-start-end-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-se-md</td><td>border-start-end-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-se-lg</td><td>border-start-end-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-se-xl</td><td>border-start-end-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-se-2xl</td><td>border-start-end-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-se-3xl</td><td>border-start-end-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-se-full</td><td>border-start-end-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-ee-none</td><td>border-end-end-radius: 0px;</td><td><div style="border-top-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-ee-xs</td><td>border-end-end-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-ee-sm</td><td>border-end-end-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-ee-md</td><td>border-end-end-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-ee-lg</td><td>border-end-end-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-ee-xl</td><td>border-end-end-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-ee-2xl</td><td>border-end-end-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-ee-3xl</td><td>border-end-end-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-ee-full</td><td>border-end-end-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-es-none</td><td>border-end-start-radius: 0px;</td><td><div style="border-top-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-es-xs</td><td>border-end-start-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-es-sm</td><td>border-end-start-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-es-md</td><td>border-end-start-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-es-lg</td><td>border-end-start-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-es-xl</td><td>border-end-start-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-es-2xl</td><td>border-end-start-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-es-3xl</td><td>border-end-start-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-es-full</td><td>border-end-start-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-tl-none</td><td>border-top-left-radius: 0px;</td><td><div style="border-top-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-tl-xs</td><td>border-top-left-radius: 0.125rem;</td><td><div style="border-top-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-tl-sm</td><td>border-top-left-radius: 0.25rem;</td><td><div style="border-top-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-tl-md</td><td>border-top-left-radius: 0.375rem;</td><td><div style="border-top-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-tl-lg</td><td>border-top-left-radius: 0.5rem;</td><td><div style="border-top-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-tl-xl</td><td>border-top-left-radius: 0.75rem;</td><td><div style="border-top-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-tl-2xl</td><td>border-top-left-radius: 1rem;</td><td><div style="border-top-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-tl-3xl</td><td>border-top-left-radius: 1.5rem;</td><td><div style="border-top-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-tl-full</td><td>border-top-left-radius: 9999px;</td><td><div style="border-top-left-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-tr-none</td><td>border-top-right-radius: 0px;</td><td><div style="border-top-right-radius: 0px;"></div></td></tr>
     <tr><td>rounded-tr-xs</td><td>border-top-right-radius: 0.125rem;</td><td><div style="border-top-right-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-tr-sm</td><td>border-top-right-radius: 0.25rem;</td><td><div style="border-top-right-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-tr-md</td><td>border-top-right-radius: 0.375rem;</td><td><div style="border-top-right-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-tr-lg</td><td>border-top-right-radius: 0.5rem;</td><td><div style="border-top-right-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-tr-xl</td><td>border-top-right-radius: 0.75rem;</td><td><div style="border-top-right-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-tr-2xl</td><td>border-top-right-radius: 1rem;</td><td><div style="border-top-right-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-tr-3xl</td><td>border-top-right-radius: 1.5rem;</td><td><div style="border-top-right-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-tr-full</td><td>border-top-right-radius: 9999px;</td><td><div style="border-top-right-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-br-none</td><td>border-bottom-right-radius: 0px;</td><td><div style="border-bottom-right-radius: 0px;"></div></td></tr>
     <tr><td>rounded-br-xs</td><td>border-bottom-right-radius: 0.125rem;</td><td><div style="border-bottom-right-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-br-sm</td><td>border-bottom-right-radius: 0.25rem;</td><td><div style="border-bottom-right-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-br-md</td><td>border-bottom-right-radius: 0.375rem;</td><td><div style="border-bottom-right-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-br-lg</td><td>border-bottom-right-radius: 0.5rem;</td><td><div style="border-bottom-right-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-br-xl</td><td>border-bottom-right-radius: 0.75rem;</td><td><div style="border-bottom-right-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-br-2xl</td><td>border-bottom-right-radius: 1rem;</td><td><div style="border-bottom-right-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-br-3xl</td><td>border-bottom-right-radius: 1.5rem;</td><td><div style="border-bottom-right-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-br-full</td><td>border-bottom-right-radius: 9999px;</td><td><div style="border-bottom-right-radius: 9999px;"></div></td></tr>
     <tr><td>rounded-bl-none</td><td>border-bottom-left-radius: 0px;</td><td><div style="border-bottom-left-radius: 0px;"></div></td></tr>
     <tr><td>rounded-bl-xs</td><td>border-bottom-left-radius: 0.125rem;</td><td><div style="border-bottom-left-radius: 0.125rem;"></div></td></tr>
     <tr><td>rounded-bl-sm</td><td>border-bottom-left-radius: 0.25rem;</td><td><div style="border-bottom-left-radius: 0.25rem;"></div></td></tr>
     <tr><td>rounded-bl-md</td><td>border-bottom-left-radius: 0.375rem;</td><td><div style="border-bottom-left-radius: 0.375rem;"></div></td></tr>
     <tr><td>rounded-bl-lg</td><td>border-bottom-left-radius: 0.5rem;</td><td><div style="border-bottom-left-radius: 0.5rem;"></div></td></tr>
     <tr><td>rounded-bl-xl</td><td>border-bottom-left-radius: 0.75rem;</td><td><div style="border-bottom-left-radius: 0.75rem;"></div></td></tr>
     <tr><td>rounded-bl-2xl</td><td>border-bottom-left-radius: 1rem;</td><td><div style="border-bottom-left-radius: 1rem;"></div></td></tr>
     <tr><td>rounded-bl-3xl</td><td>border-bottom-left-radius: 1.5rem;</td><td><div style="border-bottom-left-radius: 1.5rem;"></div></td></tr>
     <tr><td>rounded-bl-full</td><td>border-bottom-left-radius: 9999px;</td><td><div style="border-bottom-left-radius: 9999px;"></div></td></tr>
  </tbody>
</table>

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property is allowed as arbitrary value.
For example, `rounded-bl-[12px]`.

[Tailwind reference](https://tailwindcss.com/docs/border-radius)