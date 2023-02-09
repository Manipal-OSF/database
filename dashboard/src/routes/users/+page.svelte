<script lang="ts">
	import type { PageData } from "./$types";

    export let data: PageData;
</script>

<div class="overflow-x-auto">
<table class="table w-full">
    <!-- head -->
    <thead>
    <tr>
        <th></th>
        <th>{data.registrationNumber}</th>
        <th>Job</th>
        <th>Favorite Color</th>
    </tr>
    </thead>
    <tbody>
    <!-- row 1 -->
    <tr>
        <th>1</th>
        <td>Cy Ganderton</td>
        <td>Quality Control Specialist</td>
        <td>Blue</td>
    </tr>
    <!-- row 2 -->
    <tr class="hover">
        <th>2</th>
        <td>Hart Hagerty</td>
        <td>Desktop Support Technician</td>
        <td>Purple</td>
    </tr>
    <!-- row 3 -->
    <tr>
        <th>3</th>
        <td>Brice Swyre</td>
        <td>Tax Accountant</td>
        <td>Red</td>
    </tr>
    </tbody>
</table>
</div>
