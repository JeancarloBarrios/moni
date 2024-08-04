document.addEventListener('DOMContentLoaded', () => {
    const dialogContainer = document.getElementById('dialog-container');

    dialogContainer.addEventListener('htmx:afterOnLoad', () => {
        const dialog = document.getElementById('addToReportDialog');
        const closeButton = document.getElementById('closeDialogButton');

        dialog.show();

        closeButton.addEventListener('click', () => dialog.hide());
    });
});
